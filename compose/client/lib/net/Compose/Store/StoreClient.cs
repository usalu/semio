#nullable enable

using System;
using System.Diagnostics;
using System.IO;
using System.Net;
using System.Net.Http;
using System.Net.Sockets;
using System.Text;
using System.Threading;

#region 🔌Adapters
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
#endregion 🔌Adapters

namespace Compose.Store;

/// <summary>🌐 Thin HTTP GraphQL client to <c>compose-store</c> (<c>POST /install</c>, <c>POST /graphql</c>); same wire as <c>compose/js</c> <see cref="StoreSession.OpenHttp"/>.</summary>
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

    public void Start()
    {
        if (_baseUrl != null) return;
        if (_process != null) return;
        if (!System.IO.File.Exists(_binaryPath))
            throw new FileNotFoundException("compose-store binary not found", _binaryPath);

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
        psi.Environment["COMPOSE_STORE_PORT"] = port.ToString();
        var rl = Environment.GetEnvironmentVariable("RUST_LOG");
        psi.Environment["RUST_LOG"] = string.IsNullOrEmpty(rl) ? "error" : rl!;

        _process = Process.Start(psi) ?? throw new IOException("compose-store: start failed");
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
                throw new IOException("compose-store exited before ready");
            var line = process.StandardOutput.ReadLine();
            if (line == null)
            {
                Thread.Sleep(10);
                continue;
            }
            if (line.Length == 0) continue;
            try
            {
                var o = ComposeJson.Codec.ParseJsonRoot(line) as JObject;
                if (o["composeStoreReady"]?.Value<bool>() == true)
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
        throw new TimeoutException("compose-store: ready line timeout");
    }

    private string BaseUrl
    {
        get
        {
            Start();
            return _baseUrl ?? throw new InvalidOperationException("compose-store: no base url");
        }
    }

    /// <summary>📦 <c>POST /install</c> — exactly one install field per <c>compose-store</c>.</summary>
    internal void Install(JObject body)
    {
        var json = body.ToString(Formatting.None);
        using var content = new StringContent(json, Encoding.UTF8, "application/json");
        var r = _http.PostAsync($"{BaseUrl}/install", content).GetAwaiter().GetResult();
        var t = r.Content.ReadAsStringAsync().GetAwaiter().GetResult();
        if (!r.IsSuccessStatusCode)
            throw new IOException($"compose-store install {(int)r.StatusCode}: {t}");
        WarmGraphqlSession();
    }

    /// <summary>🧾 Warm-path after install — <c>session.start</c> + store cursor probe (compose/js <c>warmGraphqlRead</c>).</summary>
    internal void WarmGraphqlSession()
    {
        try
        {
            ExecuteMutation(StoreGraphql.SessionStartMutation());
        }
        catch
        {
            /* session may already be started */
        }
        _ = ExecuteQuery(StoreGraphql.SessionStoresCursorsQuery());
    }

    /// <summary>🪢 First <c>Session.stores.edges[].cursor</c> (store command scope id, typically <c>e0</c>).</summary>
    internal string DefaultStoreId()
    {
        var data = ExecuteQuery(StoreGraphql.SessionStoresCursorsQuery());
        var id = StoreGraphqlJson.DefaultStoreCursor(data);
        if (string.IsNullOrEmpty(id))
            throw new IOException("graphql: no session store cursor");
        return id;
    }

    /// <summary>📖 <c>POST /graphql</c> query root (<c>type Query</c>).</summary>
    internal JToken ExecuteQuery(string query, JObject? variables = null, string? operationName = null)
    {
        StoreGraphqlWire.AssertOperationKind(query, "query");
        return StoreGraphqlWire.UnwrapData(PostGraphql(StoreGraphqlWire.PostBodyJson(query, variables, operationName)));
    }

    /// <summary>✍️ <c>POST /graphql</c> mutation root (<c>type Mutation</c>).</summary>
    internal JToken ExecuteMutation(string query, JObject? variables = null, string? operationName = null)
    {
        StoreGraphqlWire.AssertOperationKind(query, "mutation");
        return StoreGraphqlWire.UnwrapData(PostGraphql(StoreGraphqlWire.PostBodyJson(query, variables, operationName)));
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

/// <summary>🧭 Thin GraphQL session over <see cref="StoreClient" /> (aligned with compose/js <c>Session</c> store surface).</summary>
public sealed class StoreSession : IDisposable
{
    private readonly StoreClient _client;
    private string? _storeId;
    private string? _activeChangeId;
    private WipKit? _kit;

    public StoreSession(StoreClient client)
    {
        _client = client;
        Events = new StoreEventBus();
    }

    /// <summary>📡 Command and field-change bus for this session.</summary>
    public StoreEventBus Events { get; }

    /// <summary>📦 WIP kit under <c>session.stores → wip.theKit.kit</c>.</summary>
    public WipKit Kit => _kit ??= new WipKit(this);

    /// <summary>🌐 Opens against an existing <c>compose-store</c> base URL (optional install-create first).</summary>
    public static StoreSession OpenHttp(string baseUrl, JObject? installCreateDto = null)
    {
        var c = new StoreClient(baseUrl: baseUrl);
        var session = new StoreSession(c);
        if (installCreateDto != null)
            session.InstallCreate(installCreateDto);
        else
            c.WarmGraphqlSession();
        return session;
    }

    /// <summary>🪢 Store command scope id (<c>Session.stores.edges[].cursor</c>, typically <c>e0</c>).</summary>
    public string StoreId => _storeId ??= _client.DefaultStoreId();

    //#region 🎬 install commands
    /// <summary>📦 <c>POST /install</c> with <c>create.dto</c>.</summary>
    public void InstallCreate(JObject dto) =>
        _client.Install(new JObject { ["create"] = new JObject { ["dto"] = dto } });

    /// <summary>📥 <c>POST /install</c> with normalized initial-kit projection (<see cref="StoreKitIO.KitToInstallProjection"/>).</summary>
    public void InstallProjection(Kit kit) =>
        InstallCreate(StoreKitIO.KitToInstallProjection(kit));

    /// <summary>📦 <c>POST /install</c> with <c>importFile.path</c>.</summary>
    public void InstallImportFile(string path) =>
        _client.Install(new JObject { ["importFile"] = new JObject { ["path"] = Path.GetFullPath(path) } });
    //#endregion 🎬 install commands

    //#region 🎬 session commands
    /// <summary>🎬 <c>session.store.theKit.startNewChange</c>.</summary>
    public string StartNewChange()
    {
        _activeChangeId = MutateStartNewChange();
        Events.AfterCommand();
        return _activeChangeId;
    }

    internal string EnsureChangeId() => _activeChangeId ??= MutateStartNewChange();
    //#endregion 🎬 session commands

    private string MutateStartNewChange()
    {
        var data = _client.ExecuteMutation(
            StoreGraphql.SessionStoreStartNewChangeMutation(),
            new JObject { ["storeId"] = StoreId });
        var node = data.SelectToken("session.store.theKit.startNewChange");
        StoreGraphqlJson.AssertResponseOk(node, "startNewChange");
        var changeId = StoreGraphqlJson.ResponseResultId(node as JObject);
        if (string.IsNullOrEmpty(changeId))
            throw new IOException("graphql: startNewChange returned empty change id");
        return changeId;
    }

    internal JToken ReadSessionEntry() =>
        _client.ExecuteQuery(StoreGraphql.KitSessionQueryEntry);

    internal JToken ReadKitSelection(string selection) =>
        _client.ExecuteQuery(StoreGraphql.KitSessionWipKitQuery(selection));

    internal JObject? ReadKitObject(string selection) =>
        StoreGraphqlJson.WipTheKitKit(ReadKitSelection(selection), StoreId);

    internal JToken ReadMaterialization() =>
        _client.ExecuteQuery(StoreGraphql.KitWipMaterializationQuery());

    internal void RunKitMutation(string changeId, string kitSelection, string? fieldEventKind)
    {
        var (query, variables) = StoreGraphql.ScopedKitMutation(StoreId, changeId, kitSelection);
        var data = _client.ExecuteMutation(query, variables);
        var op = kitSelection.Trim().Split('(')[0].Trim();
        var node = StoreGraphqlJson.FindKitCommandResponse(data)
            ?? data.SelectToken($"session.store.theKit.unsavedChange.kit.{op}");
        StoreGraphqlJson.AssertResponseOk(node, op);
        Events.AfterCommand(fieldEventKind);
    }

    public void Dispose() => _client.Dispose();
}

public static class StorePaths
{
    public static string ResolveStoreBinary()
    {
        var env = Environment.GetEnvironmentVariable("COMPOSE_STORE_BIN");
        if (!string.IsNullOrWhiteSpace(env) && System.IO.File.Exists(env)) return env!.Trim();
        var nextTo = Path.Combine(AppContext.BaseDirectory, "compose-store.exe");
        if (System.IO.File.Exists(nextTo)) return nextTo;
        var nextToNix = Path.Combine(AppContext.BaseDirectory, "compose-store");
        if (System.IO.File.Exists(nextToNix)) return nextToNix;
        for (var here = new DirectoryInfo(AppContext.BaseDirectory); here != null; here = here.Parent)
        {
            var win = Path.Combine(here.FullName, "target", "release", "compose-store.exe");
            if (System.IO.File.Exists(win)) return win;
            var unix = Path.Combine(here.FullName, "target", "release", "compose-store");
            if (System.IO.File.Exists(unix)) return unix;
        }
        if (System.IO.File.Exists("compose-store.exe")) return "compose-store.exe";
        if (System.IO.File.Exists("compose-store")) return "compose-store";
        return "compose-store";
    }
}
