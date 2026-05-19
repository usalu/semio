#nullable enable

using System;
using Newtonsoft.Json.Linq;

namespace Semio.Store;

/// <summary>📡 Placeholder for kit-store push events; HTTP GraphQL has no NDJSON event stream yet (see <c>semio/js</c> subscriptions).</summary>
public static class StoreEventBridge
{
    public static void Subscribe(StoreClient client, Action<JObject> onEvent) =>
        client.OnEvent += j => onEvent(j);
}
