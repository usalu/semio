// #region 🔖Header
// 💻 semio/algorithms/native-bridges/csharp/Program.cs
// Specs: Read JSON op+payload from stdin; write JSON {ok,result,error} to stdout.
// Summary: C# native bridge for algorithms Storybook proxy using semio/net Semio library only.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using Semio;

static class NativeBridge
{
    private sealed class BridgeRequest
    {
        [JsonProperty("op")] public string Op { get; set; } = "";
        [JsonProperty("kit")] public JToken Kit { get; set; } = new JObject();
        [JsonProperty("design")] public JToken Design { get; set; } = new JObject();
        [JsonProperty("designGuid")] public string DesignGuid { get; set; } = "";
        [JsonProperty("pieceGuids")] public List<string> PieceGuids { get; set; } = new();
        [JsonProperty("connectionGuids")] public List<string> ConnectionGuids { get; set; } = new();
    }

    private sealed class BridgeResponse
    {
        [JsonProperty("ok")] public bool Ok { get; set; }
        [JsonProperty("result")] public JToken? Result { get; set; }
        [JsonProperty("error")] public string? Error { get; set; }
    }

    public static int Main()
    {
        try
        {
            var input = Console.In.ReadToEnd();
            var req = JsonConvert.DeserializeObject<BridgeRequest>(input);
            if (req == null) throw new Exception("parse request: null");

            var kit = req.Kit.ToObject<Kit>();
            if (kit == null) throw new Exception("parse kit: null");

            switch (req.Op)
            {
                case "flatten":
                {
                    var diff = Kit.FlattenDesign(kit, req.DesignGuid);
                    WriteOk(JToken.FromObject(diff));
                    return 0;
                }
                case "delete":
                {
                    var design = req.Design.ToObject<Design>();
                    if (design == null) throw new Exception("parse design: null");
                    var diff = Design.DeletePiecesAndConnectionsInDesign(kit, design, req.PieceGuids ?? new List<string>(), req.ConnectionGuids ?? new List<string>());
                    WriteOk(JToken.FromObject(diff));
                    return 0;
                }
                default:
                    WriteErr("unknown op: " + req.Op);
                    return 0;
            }
        }
        catch (Exception e)
        {
            WriteErr(e.Message);
            return 0;
        }
    }

    private static void WriteOk(JToken result)
    {
        var resp = new BridgeResponse { Ok = true, Result = result, Error = null };
        Console.Out.WriteLine(JsonConvert.SerializeObject(resp));
    }

    private static void WriteErr(string msg)
    {
        var resp = new BridgeResponse { Ok = false, Result = null, Error = msg };
        Console.Out.WriteLine(JsonConvert.SerializeObject(resp));
    }
}

