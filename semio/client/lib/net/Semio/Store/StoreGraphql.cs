#nullable enable

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Semio.Store;

//#region 🌐Wire
/// <summary>🌐 GraphQL-over-HTTP wire helpers aligned with <c>semio/js</c> (<c>graphqlWirePostBodyJson</c>, operation-kind guard).</summary>
public static class StoreGraphqlWire
{
    /// <summary>🧵 Canonical POST body: <c>query</c>, <c>variables</c>, <c>operationName</c> always present.</summary>
    public static string PostBodyJson(string query, JObject? variables = null, string? operationName = null) =>
        JsonConvert.SerializeObject(new JObject
        {
            ["query"] = query,
            ["variables"] = variables ?? new JObject(),
            ["operationName"] = operationName == null ? JValue.CreateNull() : operationName,
        }, Formatting.None);

    /// <summary>🛑 Enforces golden-schema split: <c>Query</c> vs <c>Mutation</c> roots only.</summary>
    public static void AssertOperationKind(string document, string kind)
    {
        var rest = document.TrimStart();
        for (; ; )
        {
            if (rest.StartsWith("#"))
            {
                var nl = rest.IndexOf('\n');
                if (nl < 0) throw new IOException($"graphql: expected {kind}, got unknown");
                rest = rest[(nl + 1)..].TrimStart();
                continue;
            }
            break;
        }
        var head = rest.Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries).FirstOrDefault() ?? "";
        var op = head.Split('(')[0];
        if (!string.Equals(op, kind, StringComparison.OrdinalIgnoreCase))
            throw new IOException($"graphql: expected {kind}, got {head}");
    }

    /// <summary>📬 Unwraps <c>data</c> or throws on GraphQL <c>errors</c>.</summary>
    public static JToken UnwrapData(JToken response)
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
}
//#endregion 🌐Wire

//#region 🌐Documents
/// <summary>🌐 Golden-schema GraphQL documents aligned with <c>semio/js</c> (<c>schema.golden.graphql</c>).</summary>
public static class StoreGraphql
{
    /// <summary>📬 <c>Response</c> selection on command mutation leaves.</summary>
    public const string ResponseSelection =
        "ok errors { kind message requestId } result { ... on IdResult { value } }";

    /// <summary>🧭 Store entry query — mirrors <c>KIT_SESSION_QUERY_ENTRY</c> in semio/js.</summary>
    public const string KitSessionQueryEntry =
        "query KitStoreEntry { session { stores { edges { node { wip { id theKit { id } } } } } } }";

    /// <summary>🧵 GraphQL string literal for variables.</summary>
    public static string GqlString(string s) => JsonConvert.SerializeObject(s);

    /// <summary>🧵 GraphQL ID list literal.</summary>
    public static string GqlIdList(IEnumerable<string> ids) =>
        "[" + string.Join(", ", ids.Select(GqlString)) + "]";

    /// <summary>📖 <c>session.stores → wip.theKit.kit</c> read (theKit head).</summary>
    public static string KitSessionWipKitQuery(string kitSelection) =>
        $"query KitSessionWipStore {{ session {{ stores {{ edges {{ cursor node {{ wip {{ theKit {{ kit {{ {kitSelection} }} }} }} }} }} }} }} }}";

    /// <summary>📖 <c>session.stores → node</c> store branch read.</summary>
    public static string SessionStoreNodeQuery(string innerOnStore) =>
        $"query Stores {{ session {{ stores {{ edges {{ cursor node {{ {innerOnStore} }} }} }} }} }}";

    /// <summary>📖 WIP materialization: <c>initialKit</c>, <c>theKit.kit</c>, checkpoint anchors.</summary>
    public static string KitWipMaterializationQuery() =>
        @"query KitMaterialization {
  session {
    stores {
      edges {
        node {
          wip {
            initialKit { name }
            theKit {
              kit {
                name
                designs { edges { node { name } } }
                types { edges { node { name } } }
              }
            }
            checkpoints {
              edges {
                node {
                  initial { name }
                  kit { name }
                }
              }
            }
          }
        }
      }
    }
  }
}";

    public static string SessionStoresCursorsQuery() =>
        "query SessionStoreCursors { session { stores { edges { cursor } } } }";

    public static string SessionStartMutation() =>
        $"mutation SessionStart {{ session {{ start {{ {ResponseSelection} }} }} }}";

    public static string SessionEndMutation() =>
        $"mutation SessionEnd {{ session {{ end {{ {ResponseSelection} }} }} }}";

    public static string SessionStoreStartNewChangeMutation() =>
        $"mutation($storeId: ID!) {{ session {{ store(id: $storeId) {{ theKit {{ startNewChange {{ {ResponseSelection} }} }} }} }} }}";

    /// <summary>✍️ <c>session.store → theKit → unsavedChange → kit</c> scoped mutation.</summary>
    public static (string Query, JObject Variables) ScopedKitMutation(string storeId, string changeId, string kitSelection)
    {
        var inner = WithResponseSelection(kitSelection);
        return (
            $"mutation($storeId: ID!, $changeId: ID!) {{ session {{ store(id: $storeId) {{ theKit {{ unsavedChange(id: $changeId) {{ kit {{ {inner} }} }} }} }} }} }}",
            new JObject { ["storeId"] = storeId, ["changeId"] = changeId });
    }

    /// <summary>✍️ <c>kit.rename</c> on the open unsaved change.</summary>
    public static (string Query, JObject Variables) RenameKitMutation(string storeId, string changeId, string newName) =>
        ScopedKitMutation(storeId, changeId, $"rename(newName: {GqlString(newName)})");

    /// <summary>📬 Appends {@link ResponseSelection} to the innermost kit command field (mirrors semio/js <c>withResponseSelection</c>).</summary>
    public static string WithResponseSelection(string kitSelection) =>
        StoreGraphqlSelection.WithResponse(kitSelection, ResponseSelection);
}
//#endregion 🌐Documents

//#region 🌐JsonPaths
/// <summary>🧩 JSON-path helpers for golden <c>session → stores → wip → theKit → kit</c> reads.</summary>
public static class StoreGraphqlJson
{
    public static string WipPath(int storeIndex = 0) => $"session.stores.edges[{storeIndex}].node.wip";

    public static string? DefaultStoreCursor(JToken data, int storeIndex = 0) =>
        data.SelectToken($"session.stores.edges[{storeIndex}].cursor")?.Value<string>();

    public static JObject? SessionStoreNode(JToken data, string? storeCursor = null, int storeIndex = 0)
    {
        if (!string.IsNullOrEmpty(storeCursor))
        {
            var edges = data.SelectToken("session.stores.edges") as JArray;
            if (edges != null)
            {
                foreach (var e in edges)
                {
                    if (e is not JObject edge) continue;
                    if (edge["cursor"]?.Value<string>() == storeCursor)
                        return edge["node"] as JObject;
                }
            }
            return null;
        }
        return data.SelectToken($"session.stores.edges[{storeIndex}].node") as JObject;
    }

    public static JObject? WipBranch(JToken data, string? storeCursor = null, int storeIndex = 0) =>
        SessionStoreNode(data, storeCursor, storeIndex)?["wip"] as JObject;

    public static JObject? WipTheKitKit(JToken data, string? storeCursor = null, int storeIndex = 0) =>
        WipBranch(data, storeCursor, storeIndex)?["theKit"]?["kit"] as JObject;

    public static JToken? WipTheKitKitScalar(JToken data, string field, string? storeCursor = null, int storeIndex = 0) =>
        WipTheKitKit(data, storeCursor, storeIndex)?[field];

    public static JToken? WipInitialKitScalar(JToken data, string field, int storeIndex = 0) =>
        data.SelectToken($"{WipPath(storeIndex)}.initialKit.{field}");

    public static string? StartNewChangeId(JToken mutationData) =>
        mutationData.SelectToken("session.store.theKit.startNewChange.result.value")?.Value<string>();

    public static string? ResponseResultId(JToken? responseNode) =>
        responseNode?["result"]?["value"]?.Value<string>();

    /// <summary>📬 Throws when <c>Response.ok</c> is false.</summary>
    public static void AssertResponseOk(JToken? responseNode, string label)
    {
        if (responseNode is not JObject o) return;
        if (o["ok"]?.Value<bool>() == false)
        {
            var msg = o["errors"]?["message"]?.Value<string>() ?? "command failed";
            throw new IOException($"graphql: {label}: {msg}");
        }
    }

    /// <summary>📬 Finds the first <c>Response</c> node under <c>unsavedChange.kit</c> (supports aliased design commands).</summary>
    public static JToken? FindKitCommandResponse(JToken mutationData)
    {
        var kit = mutationData.SelectToken("session.store.theKit.unsavedChange.kit");
        return kit == null ? null : FindResponsePayload(kit);
    }

    static JToken? FindResponsePayload(JToken node)
    {
        if (node is JObject o && o.ContainsKey("ok")) return o;
        if (node is not JContainer container) return null;
        foreach (var child in container.Children())
        {
            var hit = FindResponsePayload(child);
            if (hit != null) return hit;
        }
        return null;
    }
}
//#endregion 🌐JsonPaths
