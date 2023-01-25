//using System;
//using System.Collections.Generic;
//using System.Linq;
//using System.Text;
//using System.Text.Json;
//using System.Text.Json.Serialization;
//using System.Threading.Tasks;
//using GH_IO.Serialization;
//using Google.Protobuf.WellKnownTypes;
//using Grasshopper.Kernel;
//using Grasshopper.Kernel.Types;
//using Rhino.Geometry;
//using Semio.Model.V1;
//using Semio.UI.Grasshopper.Utility;
//using Point = Rhino.Geometry.Point;

//namespace Semio.UI.Grasshopper.Goos
//{
//    public class ParametersGoo : GH_Goo<Dictionary<string, string>>
//    {

//        public ParametersGoo()
//        {
//            Value = new Dictionary<string, string>();
//        }
//        public ParametersGoo(Dictionary<string, string> parameters)
//        {
//            Value = parameters;
//        }

//        public override IGH_Goo Duplicate()
//        {
//            return new ParametersGoo(Value.ToDictionary(entry => entry.Key, entry => entry.Value));
//        }

//        public override string ToString() => Value.ToString();

//        public override bool CastFrom(object source)
//        {
//            if (source == null) return false;

//            return false;
//        }

//        public override bool IsValid => true;
//        public override string TypeName => "Parameters";
//        public override string TypeDescription => "Semio Parameters.";

//        public override bool Write(GH_IWriter writer)
//        {
//            writer.SetString("parameters", JsonSerializer.Serialize(Value));
//            return true;
//        }
//        public override bool Read(GH_IReader reader)
//        {
//            Value = JsonSerializer.Deserialize<Dictionary<string, string>>(reader.GetString("parameters"));
//            return true;
//        }

//    }
//}
