#nullable enable

using System;
using System.IO;
using Newtonsoft.Json.Linq;
using Semio.Store;
using Xunit;

public sealed class StoreClientTests
{
    [Fact]
    public void Semio_Store_Graphql_Smoke()
    {
        if (!File.Exists(StorePaths.ResolveStoreBinary()) &&
            !File.Exists(Path.Combine("target", "release", "semio-store.exe")) &&
            !File.Exists(Path.Combine("target", "release", "semio-store")))
        {
            return;
        }

        using var c = new StoreClient();
        var id = "00000000-0000-7000-8000-000000000099";
        var dto = new JObject
        {
            ["id"] = id,
            ["name"] = "net-store-test",
            ["types"] = new JArray(),
            ["designs"] = new JArray(),
        };
        c.Install(new JObject { ["create"] = new JObject { ["dto"] = dto } });
        var data = c.ExecuteQuery(
            "query KitWipName { session { stores { edges { node { wip { theKit { kit { name } } } } } } } } }");
        var name = data.SelectToken("session.stores.edges[0].node.wip.theKit.kit.name")?.Value<string>();
        Assert.Equal("net-store-test", name);
    }
}
