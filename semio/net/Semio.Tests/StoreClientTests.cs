#nullable enable

using System;
using System.IO;
using Newtonsoft.Json.Linq;
using Semio.Store;
using Xunit;

public sealed class StoreClientTests
{
    [Fact]
    public void Semio_Store_Sidecar_Smoke()
    {
        if (!File.Exists(StorePaths.ResolveStoreBinary()) &&
            !File.Exists(Path.Combine("target", "release", "semio-store.exe")) &&
            !File.Exists(Path.Combine("target", "release", "semio-store")))
        {
            // CI may not have Rust artifacts
            return;
        }

        using var c = new StoreClient();
        var s = c.Call("semio.generateId", new JObject());
        var id = s.Value<string>();
        Assert.False(string.IsNullOrEmpty(id));
        var dto = new JObject
        {
            ["id"] = id!,
            ["name"] = "net-store-test",
            ["types"] = new JArray(),
            ["designs"] = new JArray(),
        };
        c.Call("kit.create", new JObject { ["dto"] = dto });
        var snap = c.Call("kit.snapshot", new JObject());
        Assert.Equal("net-store-test", snap["name"]?.Value<string>());
    }
}
