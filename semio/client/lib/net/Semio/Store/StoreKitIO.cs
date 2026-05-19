#nullable enable

using System;
using System.IO;
using System.IO.Compression;
using Newtonsoft.Json.Linq;
using Formatting = Newtonsoft.Json.Formatting;

using Semio;

namespace Semio.Store;

/// <summary>📦 Load/save kits through <c>semio-store</c> GraphQL (<c>POST /install</c> + <c>POST /graphql</c>); equality via normalized JSON compare.</summary>
public static class StoreKitIO
{
    public static JObject KitToJObject(Kit kit) => JObject.Parse(Utility.Serialize(kit));

    private static Kit SnapshotToKit(JToken tok) =>
        Utility.DeserializeKit(tok.ToString(Formatting.None))!;

    private static bool TryOpenStore(out StoreClient? client)
    {
        var bin = StorePaths.ResolveStoreBinary();
        if (string.IsNullOrEmpty(bin) || !System.IO.File.Exists(bin))
        {
            client = null;
            return false;
        }
        client = new StoreClient(bin);
        return true;
    }

    private static void InstallCreate(StoreClient c, JObject dto) =>
        c.Install(new JObject { ["create"] = new JObject { ["dto"] = dto } });

    private static void InstallImportFile(StoreClient c, string path) =>
        c.Install(new JObject { ["importFile"] = new JObject { ["path"] = Path.GetFullPath(path) } });

    private static string? ResolveKitJsonPath(string folderPath)
    {
        var root = Path.GetFullPath(folderPath);
        var direct = Path.Combine(root, "kit.json");
        if (System.IO.File.Exists(direct)) return direct;
        foreach (var f in Directory.EnumerateFiles(root, "kit.json", SearchOption.AllDirectories))
        {
            if (f.Contains($"{Path.DirectorySeparatorChar}.semio{Path.DirectorySeparatorChar}", StringComparison.Ordinal)
                || f.Contains("/.semio/", StringComparison.Ordinal))
                continue;
            return f;
        }
        return null;
    }

    public static bool KitsEqual(Kit a, Kit b) =>
        JToken.DeepEquals(
            JObject.Parse(Utility.Serialize(a)),
            JObject.Parse(Utility.Serialize(b)));

    public static Kit LoadKitFromZip(string zipPath)
    {
        if (!System.IO.File.Exists(zipPath)) throw new FileNotFoundException(zipPath);
        var tempDir = Path.Combine(Path.GetTempPath(), $"semio-kit-{Guid.NewGuid()}");
        Directory.CreateDirectory(tempDir);
        try
        {
            ZipFile.ExtractToDirectory(zipPath, tempDir);
            return LoadKitFromFolder(tempDir);
        }
        finally
        {
            if (Directory.Exists(tempDir))
                Directory.Delete(tempDir, true);
        }
    }

    public static Kit LoadKitFromFolder(string folderPath)
    {
        if (!Directory.Exists(folderPath) && !System.IO.File.Exists(folderPath)) throw new IOException(folderPath);
        var kitJson = ResolveKitJsonPath(folderPath)
            ?? throw new FileNotFoundException($"No kit.json under {folderPath}");
        using var c = new StoreClient();
        InstallImportFile(c, kitJson);
        return SnapshotToKit(JObject.Parse(System.IO.File.ReadAllText(kitJson)));
    }

    public static Kit LoadKitFromFile(string filePath)
    {
        if (!System.IO.File.Exists(filePath)) throw new FileNotFoundException(filePath);
        using var c = new StoreClient();
        InstallImportFile(c, filePath);
        return SnapshotToKit(JObject.Parse(System.IO.File.ReadAllText(filePath)));
    }

    public static void SaveKitToFile(Kit kit, string filePath)
    {
        using (var c = new StoreClient())
            InstallCreate(c, KitToJObject(kit));
        System.IO.File.WriteAllText(Path.GetFullPath(filePath), Utility.Serialize(kit));
    }

    public static void SaveKitToFolder(Kit kit, string folderPath)
    {
        Directory.CreateDirectory(folderPath);
        SaveKitToFile(kit, Path.Combine(folderPath, "kit.json"));
    }

    public static void SaveKitToZip(Kit kit, string zipPath)
    {
        if (System.IO.File.Exists(zipPath)) System.IO.File.Delete(zipPath);
        var tempDir = Path.Combine(Path.GetTempPath(), $"semio-kit-{Guid.NewGuid()}");
        Directory.CreateDirectory(tempDir);
        try
        {
            SaveKitToFolder(kit, tempDir);
            ZipFile.CreateFromDirectory(tempDir, zipPath);
        }
        finally
        {
            if (Directory.Exists(tempDir))
                Directory.Delete(tempDir, true);
        }
    }
}
