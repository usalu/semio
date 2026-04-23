#nullable enable

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Threading;
using System.Threading.Tasks;
using Newtonsoft.Json.Linq;

namespace Semio.Store;

/// <summary>NDJSON JSON-RPC 2.0 to the <c>semio-store</c> process (one kit per process).</summary>
public sealed class StoreClient : IDisposable
{
    private readonly string _binaryPath;
    private Process? _process;
    private readonly object _idLock = new();
    private int _nextId = 1;
    private readonly Dictionary<int, TaskCompletionSource<JObject>> _pending = new();
    private readonly object _readPending = new();
    private StreamWriter? _stdin;
    private Thread? _readThread;
    private volatile bool _stopping;

    public StoreClient(string? binaryPath = null)
    {
        _binaryPath = string.IsNullOrWhiteSpace(binaryPath)
            ? StorePaths.ResolveStoreBinary()
            : binaryPath.Trim();
    }

    public void Start(bool noEvents = true)
    {
        if (_process != null) return;
        var psi = new ProcessStartInfo
        {
            FileName = _binaryPath,
            UseShellExecute = false,
            RedirectStandardInput = true,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true,
        };
        if (noEvents) psi.Environment["SEMIO_STORE_NO_EVENTS"] = "1";
        var rl = Environment.GetEnvironmentVariable("RUST_LOG");
        psi.Environment["RUST_LOG"] = string.IsNullOrEmpty(rl) ? "error" : rl!;

        _process = Process.Start(psi) ?? throw new IOException("semio-store: start failed");
        if (_process.HasExited) throw new IOException("semio-store exited immediately");
        _stdin = _process.StandardInput;
        _readThread = new Thread(ReadLoop) { IsBackground = true };
        _readThread.Start();
    }

    private void ReadLoop()
    {
        if (_process == null) return;
        var sr = _process.StandardOutput;
        while (!_stopping)
        {
            string? line;
            try
            {
                line = sr.ReadLine();
            }
            catch
            {
                break;
            }
            if (line == null) break;
            if (line.Length == 0) continue;
            JObject o;
            try
            {
                o = JObject.Parse(line);
            }
            catch
            {
                continue;
            }
            if (o["method"]?.Value<string>() == "event" && o["id"] == null)
            {
                if (o["params"] is JObject p) OnEvent?.Invoke(p);
                continue;
            }
            int? id = o["id"]?.Value<int?>();
            if (id is int i)
            {
                TaskCompletionSource<JObject>? tcs;
                lock (_readPending)
                {
                    if (_pending.TryGetValue(i, out tcs)) _pending.Remove(i);
                }
                tcs?.TrySetResult(o);
            }
        }
    }

    public event Action<JObject>? OnEvent;

    public JToken Call(string method, JToken? parameters)
    {
        Start();
        int id;
        lock (_idLock) id = _nextId++;
        var tcs = new TaskCompletionSource<JObject>(TaskCreationOptions.RunContinuationsAsynchronously);
        lock (_readPending) _pending[id] = tcs;
        var body = new JObject
        {
            ["jsonrpc"] = "2.0",
            ["id"] = id,
            ["method"] = method,
        };
        if (parameters != null) body["params"] = parameters;
        var line = body.ToString(Newtonsoft.Json.Formatting.None) + "\n";
        _stdin?.Write(line);
        _stdin?.Flush();
        if (!tcs.Task.Wait(TimeSpan.FromMinutes(5)))
        {
            lock (_readPending) { _pending.Remove(id); }
            throw new TimeoutException("jsonrpc: " + method);
        }
        var resp = tcs.Task.Result;
        if (resp["error"] is JObject err)
            throw new IOException("jsonrpc: " + err["code"] + " " + err["message"]);
        return resp["result"] ?? JValue.CreateNull();
    }

    public void Dispose()
    {
        _stopping = true;
        try
        {
            if (_process?.HasExited == false && _stdin != null)
            {
                _stdin.Write("{\"jsonrpc\":\"2.0\",\"id\":999999,\"method\":\"server.shutdown\"}\n");
                _stdin.Flush();
            }
        }
        catch { }

        _process?.Dispose();
        _process = null;
    }
}

public static class StorePaths
{
    public static string ResolveStoreBinary()
    {
        var env = Environment.GetEnvironmentVariable("SEMIO_STORE_BIN");
        if (!string.IsNullOrWhiteSpace(env) && System.IO.File.Exists(env)) return env!.Trim();
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
