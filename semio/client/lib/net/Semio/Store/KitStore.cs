#nullable enable

using System.IO;
using Newtonsoft.Json.Linq;

namespace Semio.Store;

/// <summary>🌐 Thin GraphQL session over <see cref="StoreClient" /> (install + wip reads / mutations).</summary>
public sealed class KitStoreSession
{
    private readonly StoreClient _c;

    public KitStoreSession(StoreClient? client = null) => _c = client ?? new StoreClient();

    public JToken Create(JObject fullDto)
    {
        _c.Install(new JObject { ["create"] = new JObject { ["dto"] = fullDto } });
        return ReadWipKitName();
    }

    public JObject Snapshot()
    {
        var data = _c.ExecuteQuery(
            "query KitWipSnapshot { session { stores { edges { node { wip { theKit { kit { id name } } } } } } } } }");
        var kit = data.SelectToken("session.stores.edges[0].node.wip.theKit.kit") as JObject
            ?? throw new IOException("graphql: missing wip.theKit.kit");
        return kit;
    }

    public JToken ExecuteChangeKitCommands(JArray commands)
    {
        _ = commands;
        throw new NotSupportedException("Use GraphQL kit command scopes (theKit → unsavedChange → kit → …) like semio/js.");
    }

    private JToken ReadWipKitName()
    {
        var data = _c.ExecuteQuery(
            "query KitWipName { session { stores { edges { node { wip { theKit { kit { name } } } } } } } } }");
        return data.SelectToken("session.stores.edges[0].node.wip.theKit.kit.name")
            ?? throw new IOException("graphql: missing wip.theKit.kit.name");
    }
}
