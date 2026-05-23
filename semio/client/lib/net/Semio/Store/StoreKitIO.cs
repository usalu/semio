#nullable enable

using System;
using System.IO;
using System.IO.Compression;
using System.Linq;
using Newtonsoft.Json.Linq;
using Formatting = Newtonsoft.Json.Formatting;

using Semio;

namespace Semio.Store;

/// <summary>📦 Load/save kits through <c>semio-store</c> GraphQL (<c>POST /install</c> + <c>POST /graphql</c>); equality via normalized JSON compare.</summary>
public static class StoreKitIO
{
    public static JObject KitToJObject(Kit kit) => JObject.Parse(Utility.Serialize(kit));

    /// <summary>📥 <c>POST /install</c> projection JSON: design pieces use <c>pose.plane</c> + <c>pose.center</c> (<c>u</c>/<c>v</c>) for semio-store hydration.</summary>
    public static JObject KitToInstallProjection(Kit kit)
    {
        var dto = KitToJObject(kit);
        if (dto["pieces"] is JArray rootPieces)
        {
            foreach (var piece in rootPieces.OfType<JObject>())
                NormalizePiecePoseForInstall(piece);
        }
        if (dto["designs"] is JArray designs)
        {
            foreach (var design in designs.OfType<JObject>())
            {
                if (design["pieces"] is not JArray pieces) continue;
                foreach (var piece in pieces.OfType<JObject>())
                    NormalizePiecePoseForInstall(piece);
            }
        }
        return dto;
    }

    private static JObject DefaultInstallPlane() => JObject.Parse(
        """{"origin":{"x":0,"y":0,"z":0},"xAxis":{"x":1,"y":0,"z":0},"yAxis":{"x":0,"y":1,"z":0}}""");

    private static JObject DefaultInstallCenter() => JObject.Parse("""{"u":0,"v":0}""");

    private static JObject NormalizeInstallCenter(JToken? center)
    {
        if (center is JObject o)
        {
            var u = o["u"] ?? o["U"];
            var v = o["v"] ?? o["V"];
            if (u != null && v != null)
                return new JObject { ["u"] = u, ["v"] = v };
        }
        return DefaultInstallCenter();
    }

    private static JObject NormalizeInstallPlane(JToken? plane)
    {
        if (plane is JObject o
            && o["origin"] is JObject origin
            && origin["x"] != null
            && (o["xAxis"] is JObject || o["x_axis"] is JObject)
            && (o["yAxis"] is JObject || o["y_axis"] is JObject))
            return (JObject)o.DeepClone();
        return DefaultInstallPlane();
    }

    private static void NormalizePiecePoseForInstall(JObject piece)
    {
        var pose = piece["pose"] as JObject;
        var planeTok = pose?["plane"] ?? piece["plane"];
        var centerTok = pose?["center"] ?? piece["center"];
        var newPose = new JObject();
        if (planeTok != null)
            newPose["plane"] = NormalizeInstallPlane(planeTok);
        else
            newPose["plane"] = DefaultInstallPlane();
        if (centerTok != null)
            newPose["center"] = NormalizeInstallCenter(centerTok);
        piece["pose"] = newPose;
        piece.Remove("plane");
        piece.Remove("center");
    }

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

    /// <summary>🧾 After install/import, verifies materialized <c>wip.theKit.kit.name</c> via golden GraphQL when store is up.</summary>
    private static void AssertGraphqlKitNameMatches(StoreSession session, string expectedName)
    {
        var actual = session.Kit.Name;
        if (actual != expectedName)
            throw new IOException($"graphql: materialized kit name {actual} != expected {expectedName}");
    }

    public static string? ResolveKitJsonPath(string folderPath)
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
        var kit = SnapshotToKit(JObject.Parse(System.IO.File.ReadAllText(kitJson)));
        if (TryOpenStore(out var c) && c != null)
        {
            using (c)
            using (var session = new StoreSession(c))
            {
                session.InstallCreate(KitToInstallProjection(kit));
                AssertGraphqlKitNameMatches(session, kit.Name ?? "");
            }
        }
        return kit;
    }

    public static Kit LoadKitFromFile(string filePath)
    {
        if (!System.IO.File.Exists(filePath)) throw new FileNotFoundException(filePath);
        var kit = SnapshotToKit(JObject.Parse(System.IO.File.ReadAllText(filePath)));
        if (TryOpenStore(out var c) && c != null)
        {
            using (c)
            using (var session = new StoreSession(c))
            {
                session.InstallCreate(KitToInstallProjection(kit));
                AssertGraphqlKitNameMatches(session, kit.Name ?? "");
            }
        }
        return kit;
    }

    public static void SaveKitToFile(Kit kit, string filePath)
    {
        if (TryOpenStore(out var c) && c != null)
        {
            using (c)
            using (var session = new StoreSession(c))
            {
                session.InstallCreate(KitToInstallProjection(kit));
                AssertGraphqlKitNameMatches(session, kit.Name ?? "");
            }
        }
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

    /// <summary>🔁 Apply <paramref name="diff"/> to <paramref name="baseKit"/> or folder snapshot, persist via <see cref="SaveKitToFolder"/>, return updated kit.</summary>
    public static Kit ApplyKitDiffAndSaveToFolder(string folderPath, KitDiff diff, Kit? baseKit = null)
    {
        var kit = Kit.ApplyDiff(baseKit ?? LoadKitFromFolder(folderPath), diff);
        SaveKitToFolder(kit, folderPath);
        return kit;
    }
}
