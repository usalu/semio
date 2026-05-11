#nullable enable

using System;
using System.IO;
using Newtonsoft.Json.Linq;
using Formatting = Newtonsoft.Json.Formatting;

using Semio;

namespace Semio.Store;

/// <summary>Load/save kits through <c>semio-store</c> JSON-RPC; equality via <c>kit.equals</c>.</summary>
public static class StoreKitIO
{
    public static JObject KitToJObject(Kit kit) => JObject.Parse(Utility.Serialize(kit));

    private static Kit SnapshotToKit(JToken tok) =>
        Utility.DeserializeKit(tok.ToString(Formatting.None))!;

    public static bool KitsEqual(Kit a, Kit b)
    {
        var p = StorePaths.ResolveStoreBinary();
        if (string.IsNullOrEmpty(p) || !System.IO.File.Exists(p))
            return JToken.DeepEquals(
                JObject.Parse(Utility.Serialize(a)),
                JObject.Parse(Utility.Serialize(b)));
        using var c = new StoreClient();
        c.Start();
        var t = c.Call("kit.equals", new JObject { ["a"] = KitToJObject(a), ["b"] = KitToJObject(b) });
        return t.Type == JTokenType.Boolean && t.Value<bool>();
    }

    public static Kit LoadKitFromZip(string zipPath)
    {
        if (!System.IO.File.Exists(zipPath)) throw new FileNotFoundException(zipPath);
        using var c = new StoreClient();
        c.Start();
        c.Call("io.importFromZip", new JObject { ["path"] = Path.GetFullPath(zipPath) });
        var tok = c.Call("kit.snapshot", new JObject());
        return SnapshotToKit(tok);
    }

    public static Kit LoadKitFromFolder(string folderPath)
    {
        if (!Directory.Exists(folderPath) && !System.IO.File.Exists(folderPath)) throw new IOException(folderPath);
        using var c = new StoreClient();
        c.Start();
        c.Call("io.importFromFolder", new JObject { ["path"] = Path.GetFullPath(folderPath) });
        var tok = c.Call("kit.snapshot", new JObject());
        return SnapshotToKit(tok);
    }

    public static Kit LoadKitFromFile(string filePath)
    {
        if (!System.IO.File.Exists(filePath)) throw new FileNotFoundException(filePath);
        using var c = new StoreClient();
        c.Start();
        c.Call("io.importFromFile", new JObject { ["path"] = Path.GetFullPath(filePath) });
        var tok = c.Call("kit.snapshot", new JObject());
        return SnapshotToKit(tok);
    }

    public static void SaveKitToFile(Kit kit, string filePath)
    {
        using var c = new StoreClient();
        c.Start();
        c.Call("kit.create", new JObject { ["dto"] = KitToJObject(kit) });
        c.Call("io.exportToFile", new JObject { ["path"] = Path.GetFullPath(filePath) });
    }

    public static void SaveKitToFolder(Kit kit, string folderPath)
    {
        using var c = new StoreClient();
        c.Start();
        c.Call("kit.create", new JObject { ["dto"] = KitToJObject(kit) });
        c.Call("io.exportToFolder", new JObject { ["path"] = Path.GetFullPath(folderPath) });
    }

    public static void SaveKitToZip(Kit kit, string zipPath)
    {
        if (System.IO.File.Exists(zipPath)) System.IO.File.Delete(zipPath);
        using var c = new StoreClient();
        c.Start();
        c.Call("kit.create", new JObject { ["dto"] = KitToJObject(kit) });
        c.Call("io.exportToZip", new JObject { ["path"] = Path.GetFullPath(zipPath) });
    }
}
