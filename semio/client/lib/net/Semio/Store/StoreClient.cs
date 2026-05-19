#nullable enable

using System;
using System.Diagnostics;
using System.IO;
using System.Net;
using System.Net.Http;
using System.Net.Sockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Semio.Store;

//#region 🌐GraphqlWire
/// <summary>🌐 GraphQL-over-HTTP POST body aligned with <c>semio/js</c> and <c>semio-store</c> (<c>query</c>, <c>variables</c>, <c>operationName</c> always present).</summary>
internal static class GraphqlWire
{
    internal static string PostBodyJson(string query, JObject? variables = null, string? operationName = null) =>
        JsonConvert.SerializeObject(new JObject
        {
            ["query"] = query,
            ["variables"] = variables ?? new JObject(),
            ["operationName"] = operationName == null ? JValue.CreateNull() : operationName,
        }, Formatting.None);

    internal static JToken UnwrapData(JToken response)
    {
        if (response is not JObject o) throw new IOException("graphql: response is not an object");
        if (o["errors"] is JArray errs && errs.Count > 0)
        {
            var msg = errs[0]?["message"]?.Value<string>() ?? "GraphQL error";
            throw new IOException("graphql: " + msg);
        }
        var data = o["data"];
        if (data == null || data.Type == JTokenType.Null) throw new IOException("graphql: no data in response");
        return data;
    }

    internal static void AssertOperationKind(string document, string kind)
    {
        var rest = document.TrimStart();
        for (; ; )
        {
            if (rest.StartsWith("#", StringComparison.Ordinal))
            {
                var nl = rest.IndexOf('\n');
                if (nl < 0) throw new IOException($"graphql: expected {kind}, got unknown");
                rest = rest[(nl + 1)..].TrimStart();
                continue;
            }
            break;
        }
        var head = rest.Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries).FirstOrDefault() ?? "";
        if (!string.Equals(head, kind, StringComparison.OrdinalIgnoreCase))
            throw new IOException($"graphql: expected {kind}, got {head}");
    }
}
//#endregion 🌐GraphqlWire

/// <summary>🌐 Thin HTTP GraphQL client to <c>semio-store</c> (<c>POST /install</c>, <c>POST /graphql</c>); same wire as <c>semio/js</c> <see cref="Session.openHttp"/>.</summary>
public sealed class StoreClient : IDisposable
{
    private readonly string _binaryPath;
    private readonly HttpClient _http;
    private Process? _process;
    private string? _baseUrl;
    public StoreClient(string? binaryPath = null, string? baseUrl = null)
    {
        _binaryPath = string.IsNullOrWhiteSpace(binaryPath)
            ? StorePaths.ResolveStoreBinary()
            : binaryPath.Trim();
        _http = new HttpClient { Timeout = TimeSpan.FromMinutes(5) };
        if (!string.IsNullOrWhiteSpace(baseUrl))
            _baseUrl = baseUrl.TrimEnd('/');
    }

    /// <summary>📡 Kit-store push events are not exposed over HTTP GraphQL yet; subscribe via GraphQL <c>subscription</c> when the sidecar adds a stream.</summary>
    public event Action<JObject>? OnEvent;

    public void Start()
    {
        if (_baseUrl != null) return;
        if (_process != null) return;
        if (!System.IO.File.Exists(_binaryPath))
            throw new FileNotFoundException("semio-store binary not found", _binaryPath);

        var port = AllocateFreeTcpPort();
        var psi = new ProcessStartInfo
        {
            FileName = _binaryPath,
            UseShellExecute = false,
            RedirectStandardInput = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true,
        };
        psi.Environment["SEMIO_STORE_PORT"] = port.ToString();
        var rl = Environment.GetEnvironmentVariable("RUST_LOG");
        psi.Environment["RUST_LOG"] = string.IsNullOrEmpty(rl) ? "error" : rl!;

        _process = Process.Start(psi) ?? throw new IOException("semio-store: start failed");
        _baseUrl = ReadReadyBaseUrl(_process, port);
    }

    private static int AllocateFreeTcpPort()
    {
        var listener = new TcpListener(IPAddress.Loopback, 0);
        listener.Start();
        try
        {
            return ((IPEndPoint)listener.LocalEndpoint).Port;
        }
        finally
        {
            listener.Stop();
        }
    }

    private static string ReadReadyBaseUrl(Process process, int fallbackPort)
    {
        var deadline = DateTime.UtcNow.AddSeconds(60);
        while (DateTime.UtcNow < deadline)
        {
            if (process.HasExited)
                throw new IOException("semio-store exited before ready");
            var line = process.StandardOutput.ReadLine();
            if (line == null)
            {
                Thread.Sleep(10);
                continue;
            }
            if (line.Length == 0) continue;
            try
            {
                var o = JObject.Parse(line);
                if (o["semioStoreReady"]?.Value<bool>() == true)
                {
                    var port = o["port"]?.Value<int?>() ?? fallbackPort;
                    return $"http://127.0.0.1:{port}";
                }
            }
            catch (JsonException)
            {
                /* not the ready line */
            }
        }
        throw new TimeoutException("semio-store: ready line timeout");
    }

    private string BaseUrl
    {
        get
        {
            Start();
            return _baseUrl ?? throw new InvalidOperationException("semio-store: no base url");
        }
    }

    /// <summary>📦 <c>POST /install</c> — exactly one of <c>create</c>, <c>importFile</c>, … per <c>semio-store</c>.</summary>
    public void Install(JObject body)
    {
        var json = body.ToString(Formatting.None);
        using var content = new StringContent(json, Encoding.UTF8, "application/json");
        var r = _http.PostAsync($"{BaseUrl}/install", content).GetAwaiter().GetResult();
        var t = r.Content.ReadAsStringAsync().GetAwaiter().GetResult();
        if (!r.IsSuccessStatusCode)
            throw new IOException($"semio-store install {(int)r.StatusCode}: {t}");
    }

    /// <summary>📖 <c>POST /graphql</c> query root (<c>type Query</c>).</summary>
    public JToken ExecuteQuery(string query, JObject? variables = null, string? operationName = null)
    {
        GraphqlWire.AssertOperationKind(query, "query");
        return GraphqlWire.UnwrapData(PostGraphql(GraphqlWire.PostBodyJson(query, variables, operationName)));
    }

    /// <summary>✍️ <c>POST /graphql</c> mutation root (<c>type Mutation</c>).</summary>
    public JToken ExecuteMutation(string query, JObject? variables = null, string? operationName = null)
    {
        GraphqlWire.AssertOperationKind(query, "mutation");
        return GraphqlWire.UnwrapData(PostGraphql(GraphqlWire.PostBodyJson(query, variables, operationName)));
    }

    private JToken PostGraphql(string requestJson)
    {
        using var content = new StringContent(requestJson, Encoding.UTF8, "application/json");
        using var req = new HttpRequestMessage(HttpMethod.Post, $"{BaseUrl}/graphql") { Content = content };
        req.Headers.TryAddWithoutValidation("Accept", "application/json");
        var r = _http.SendAsync(req).GetAwaiter().GetResult();
        var t = r.Content.ReadAsStringAsync().GetAwaiter().GetResult();
        if (!r.IsSuccessStatusCode)
            throw new IOException($"graphql http {(int)r.StatusCode}: {t}");
        return JToken.Parse(t);
    }

    public void Dispose()
    {
        try
        {
            if (_baseUrl != null)
            {
                using var _ = _http.PostAsync($"{_baseUrl}/server/shutdown", null).GetAwaiter().GetResult();
            }
        }
        catch { }

        try
        {
            if (_process?.HasExited == false)
                _process.WaitForExit(2000);
        }
        catch { }

        _process?.Dispose();
        _process = null;
        _http.Dispose();
    }
}

public static class StorePaths
{
    public static string ResolveStoreBinary()
    {
        var env = Environment.GetEnvironmentVariable("SEMIO_STORE_BIN");
        if (!string.IsNullOrWhiteSpace(env) && System.IO.File.Exists(env)) return env!.Trim();
        var nextTo = Path.Combine(AppContext.BaseDirectory, "semio-store.exe");
        if (System.IO.File.Exists(nextTo)) return nextTo;
        var nextToNix = Path.Combine(AppContext.BaseDirectory, "semio-store");
        if (System.IO.File.Exists(nextToNix)) return nextToNix;
        for (var here = new DirectoryInfo(AppContext.BaseDirectory); here != null; here = here.Parent)
        {
            var win = Path.Combine(here.FullName, "target", "release", "semio-store.exe");
            if (System.IO.File.Exists(win)) return win;
            var unix = Path.Combine(here.FullName, "target", "release", "semio-store");
            if (System.IO.File.Exists(unix)) return unix;
        }
        if (System.IO.File.Exists("semio-store.exe")) return "semio-store.exe";
        if (System.IO.File.Exists("semio-store")) return "semio-store";
        return "semio-store";
    }
}
