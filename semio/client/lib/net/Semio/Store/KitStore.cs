#nullable enable

using Newtonsoft.Json.Linq;

namespace Semio.Store;

/// <summary>Thin convenience wrapper: call sequence matches common kit workflows.</summary>
public sealed class KitStoreSession
{
    private readonly StoreClient _c;

    public KitStoreSession(StoreClient? client = null) => _c = client ?? new StoreClient();

    public JToken Create(JObject fullDto) => _c.Call("kit.create", new JObject { ["dto"] = fullDto });

    public JObject Snapshot() => (JObject)_c.Call("kit.snapshot", new JObject());

    public JToken ExecuteChangeKitCommands(JArray commands) => _c.Call("kit.executeChangeKitCommands", new JObject { ["cmds"] = commands });
}
