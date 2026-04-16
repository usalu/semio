#nullable enable

using System;
using Newtonsoft.Json.Linq;

namespace Semio.Store;

/// <summary>Subscribes to <c>event</c> JSON-RPC lines from <see cref="StoreClient" /> (when the sidecar has events enabled).</summary>
public static class StoreEventBridge
{
    public static void Subscribe(StoreClient client, Action<JObject> onEvent) =>
        client.OnEvent += (j) => onEvent(j);
}
