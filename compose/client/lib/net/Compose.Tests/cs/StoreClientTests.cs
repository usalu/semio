#nullable enable

using System;
using System.IO;
using System.Linq;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Compose;
using Compose.Store;
using Xunit;

public sealed class StoreGraphqlWireTests
{
    [Fact]
    public void GraphqlWire_PostBodyJson_Always_Carries_Query_Variables_OperationName()
    {
        var raw = StoreGraphqlWire.PostBodyJson("query Q { __typename }");
        var o = JObject.Parse(raw);
        Assert.Equal(new[] { "operationName", "query", "variables" }, o.Properties().Select(p => p.Name).OrderBy(x => x));
        Assert.Equal("query Q { __typename }", o["query"]?.Value<string>());
        Assert.NotNull(o["variables"]);
        Assert.True(o["operationName"]!.Type == JTokenType.Null);

        var withVars = JObject.Parse(StoreGraphqlWire.PostBodyJson(
            "mutation M { __typename }",
            new JObject { ["a"] = 1 },
            "M"));
        Assert.Equal("M", withVars["operationName"]?.Value<string>());
        Assert.Equal(1, withVars["variables"]?["a"]?.Value<int>());
    }

    [Fact]
    public void GraphqlWire_AssertOperationKind_Matches_Js()
    {
        StoreGraphqlWire.AssertOperationKind("  #c\nquery X { session { __typename } }", "query");
        StoreGraphqlWire.AssertOperationKind("mutation { session { start } }", "mutation");
        StoreGraphqlWire.AssertOperationKind("mutation($storeId: ID!) { session { store(id: $storeId) { id } } }", "mutation");
        Assert.Throws<IOException>(() => StoreGraphqlWire.AssertOperationKind("mutation { __typename }", "query"));
    }

    [Fact]
    public void StoreGraphql_WithResponseSelection_Nests_Like_Js()
    {
        var s = StoreGraphql.WithResponseSelection("rename(newName: \"x\")");
        Assert.Contains("rename(newName: \"x\")", s);
        Assert.Contains(StoreGraphql.ResponseSelection, s);
    }

    [Fact]
    public void StoreGraphql_WithResponseSelection_Preserves_PositionInput_Objects()
    {
        var s = StoreGraphql.WithResponseSelection(
            "design(id: \"d\") { addFixedPiece(blueprintId: \"b\", position: { center: { u: 0, v: 0 }, plane: { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } } } }) }");
        Assert.Contains("addFixedPiece", s);
        Assert.Contains("ok errors", s);
        Assert.DoesNotContain("xAxis: {  { ok", s);
    }
}

public sealed class StoreClientTests
{
    public static bool StoreBinaryPresent() =>
        System.IO.File.Exists(StorePaths.ResolveStoreBinary()) ||
        System.IO.File.Exists(Path.Combine("target", "release", "compose-store.exe")) ||
        System.IO.File.Exists(Path.Combine("target", "release", "compose-store"));

    [Fact]
    public void Compose_Store_Graphql_Entry_And_Wip_Read()
    {
        if (!StoreBinaryPresent()) return;

        using var session = CreateInstalledSession("net-store-entry", "net-store-entry");
        var entry = session.ReadSessionEntry();
        Assert.False(string.IsNullOrEmpty(entry.SelectToken("session.stores.edges[0].node.wip.id")?.Value<string>()));
        Assert.False(string.IsNullOrEmpty(entry.SelectToken("session.stores.edges[0].node.wip.theKit.id")?.Value<string>()));

        Assert.Equal("net-store-entry", session.Kit.Name);
    }

    [Fact]
    public void Compose_Store_Graphql_Rename_Materialization_Roundtrip()
    {
        if (!StoreBinaryPresent()) return;

        using var session = CreateInstalledSession("00000000-0000-7000-8000-000000000099", "SeedName");
        var changeId = session.StartNewChange();
        Assert.False(string.IsNullOrEmpty(changeId));

        session.Kit.Rename("RenamedKit");

        var mat = session.Kit.Materialization;
        var wip = StoreGraphqlJson.WipPath();
        Assert.Equal("SeedName", mat.SelectToken($"{wip}.initialKit.name")?.Value<string>());
        Assert.Equal("RenamedKit", mat.SelectToken($"{wip}.theKit.kit.name")?.Value<string>());
        Assert.Equal("SeedName", mat.SelectToken($"{wip}.checkpoints.edges[0].node.initial.name")?.Value<string>());
        Assert.Equal("RenamedKit", mat.SelectToken($"{wip}.checkpoints.edges[0].node.kit.name")?.Value<string>());
    }

    [Fact]
    public void StoreSession_InstallCreate_And_Kit_Object_Read()
    {
        if (!StoreBinaryPresent()) return;

        var id = "00000000-0000-7000-8000-0000000000aa";
        using var session = CreateInstalledSession(id, "installed-kit");
        var kit = session.Kit.Object;
        Assert.Equal(id, kit["id"]?.Value<string>());
        Assert.Equal("installed-kit", kit["name"]?.Value<string>());
    }

    [Fact]
    public void WipKit_Rename_Fires_NameChanged()
    {
        if (!StoreBinaryPresent()) return;

        using var session = CreateInstalledSession("00000000-0000-7000-8000-0000000000bb", "Before");
        string? seen = null;
        session.Kit.NameChanged += n => seen = n;
        session.Kit.Rename("After");
        Assert.Equal("After", seen);
        Assert.Equal("After", session.Kit.Name);
    }

    [Fact]
    public void Compose_Store_Graphql_CreateDesign_And_CreateType_Materialize()
    {
        if (!StoreBinaryPresent()) return;

        using var session = CreateInstalledSession("00000000-0000-7000-8000-0000000000cc", "KitWithEntities");
        Assert.False(string.IsNullOrEmpty(session.StartNewChange()));

        session.Kit.CreateDesign("layout-alpha");
        session.Kit.CreateType("kind-beta");

        var designNames = session.Kit.Object["designs"]?["edges"]?
            .Children<JObject>()
            .Select(e => e["node"]?["name"]?.Value<string>())
            .Where(n => n != null)
            .ToList();
        var typeNames = session.Kit.Object["types"]?["edges"]?
            .Children<JObject>()
            .Select(e => e["node"]?["name"]?.Value<string>())
            .Where(n => n != null)
            .ToList();
        Assert.Contains("layout-alpha", designNames!);
        Assert.Contains("kind-beta", typeNames!);

        var wip = StoreGraphqlJson.WipPath();
        var matDesignNames = session.Kit.Materialization.SelectTokens($"{wip}.theKit.kit.designs.edges[*].node.name")
            .Select(t => t.Value<string>())
            .Where(n => n != null)
            .ToList();
        var matTypeNames = session.Kit.Materialization.SelectTokens($"{wip}.theKit.kit.types.edges[*].node.name")
            .Select(t => t.Value<string>())
            .Where(n => n != null)
            .ToList();
        Assert.Contains("layout-alpha", matDesignNames);
        Assert.Contains("kind-beta", matTypeNames);
    }

    private static StoreSession CreateInstalledSession(string kitId, string kitName)
    {
        var c = new StoreClient();
        var session = new StoreSession(c);
        session.InstallCreate(new JObject
        {
            ["id"] = kitId,
            ["name"] = kitName,
            ["types"] = new JArray(),
            ["designs"] = new JArray(),
        });
        return session;
    }
}

public sealed class StoreKitIOTests
{
    [Fact]
    public void Metabolism_KitToInstallDto_Position_Centers_Are_Uv_Objects()
    {
        var kit = Compose.Tests.Tests.LoadAsset<Kit>("stores/metabolism/wip/initialKit/kit.compose.json");
        var dto = StoreKitIO.KitToInstallProjection(kit);
        static void Walk(JToken? t)
        {
            if (t is not JObject o) return;
            if (o["position"] is JObject pos) AssertCenterObject(pos);
            if (o["pose"] is JObject pose) AssertCenterObject(pose);
            foreach (var p in o.Properties()) Walk(p.Value);
        }
        static void AssertCenterObject(JObject pos)
        {
            if (!pos.TryGetValue("center", out var c) || c.Type == JTokenType.Null) return;
            Assert.True(c is JObject, $"position.center must be object, got {c.Type}: {c}");
            var co = (JObject)c;
            Assert.NotNull(co["u"] ?? co["U"]);
            Assert.NotNull(co["v"] ?? co["V"]);
        }
        Walk(dto);
    }

    [Fact]
    public void Metabolism_InstallProjection_Roundtrip_When_Store_Present()
    {
        if (!StoreClientTests.StoreBinaryPresent()) return;
        var kit = Compose.Tests.Tests.LoadAsset<Kit>("stores/metabolism/wip/initialKit/kit.compose.json");
        var dir = Path.Combine(Path.GetTempPath(), $"compose-metabolism-e2e-{Guid.NewGuid():N}");
        Directory.CreateDirectory(dir);
        try
        {
            StoreKitIO.SaveKitToFolder(kit, dir);
            var reloaded = StoreKitIO.LoadKitFromFolder(dir);
            Assert.Equal(kit.Name, reloaded.Name);
            Assert.True(reloaded.Designs.Count > 0);
        }
        finally
        {
            if (Directory.Exists(dir)) Directory.Delete(dir, true);
        }
    }

    [Fact]
    public void ApplyKitDiffAndSaveToFolder_Renames_OnDisk()
    {
        var dir = Path.Combine(Path.GetTempPath(), $"compose-kitio-{Guid.NewGuid():N}");
        Directory.CreateDirectory(dir);
        try
        {
            var seed = new Kit { Id = "00000000-0000-7000-8000-0000000000dd", Name = "Before" };
            StoreKitIO.SaveKitToFolder(seed, dir);
            var diff = new KitDiff { Name = "After" };
            var updated = StoreKitIO.ApplyKitDiffAndSaveToFolder(dir, diff);
            Assert.Equal("After", updated.Name);
            var reloaded = StoreKitIO.LoadKitFromFolder(dir);
            Assert.Equal("After", reloaded.Name);
        }
        finally
        {
            if (Directory.Exists(dir))
                Directory.Delete(dir, true);
        }
    }

    [Fact]
    public void ApplyKitDiffAndSaveToFolder_Uses_BaseKit_Without_Reload()
    {
        var dir = Path.Combine(Path.GetTempPath(), $"compose-kitio-{Guid.NewGuid():N}");
        Directory.CreateDirectory(dir);
        try
        {
            var seed = new Kit { Id = "00000000-0000-7000-8000-0000000000ee", Name = "Cached" };
            StoreKitIO.SaveKitToFolder(seed, dir);
            var cached = new Kit { Id = seed.Id, Name = "CachedInMemory" };
            var diff = new KitDiff { Name = "Merged" };
            var updated = StoreKitIO.ApplyKitDiffAndSaveToFolder(dir, diff, cached);
            Assert.Equal("Merged", updated.Name);
        }
        finally
        {
            if (Directory.Exists(dir))
                Directory.Delete(dir, true);
        }
    }
}
