//// TODO Autogenerate
//using System;
//using System.Collections.Generic;
//using System.Linq;
//using System.Runtime.Remoting.Messaging;
//using System.Text;
//using System.Threading.Tasks;
//using System.Xml.Linq;
//using Grasshopper.Kernel;
//using Grasshopper.Kernel.Types;
//using Rhino.Geometry;
//using Semio.Model.V1;
//using Encoding = Semio.Model.V1.Encoding;

//namespace Semio.UI.Grasshopper.Goos
//{
//    public class EncodingGoo : SemioGoo<Encoding>
//    {
//        public EncodingGoo()
//        {
//            Value = new Encoding();
//        }
//        public EncodingGoo(Encoding encoding)
//        {
//            Value = encoding;
//        }
//        public override IGH_Goo Duplicate() => new EncodingGoo(Value);
//        public override string TypeName => "Encoding";
//        public override string TypeDescription => "";
//        public override bool CastTo<Q>(ref Q target)
//        {
//            if (typeof(Q).IsAssignableFrom(typeof(Encoding)))
//            {
//                object ptr = Value;
//                target = (Q)ptr;
//                return true;
//            }
//            return false;
//        }

//        public override bool CastFrom(object source)
//        {
//            if (source == null) { return false; }

//            if (GH_Convert.ToString(source, out var encodingValue, GH_Conversion.Both))
//            {
//                bool success = Enum.TryParse(encodingValue, true, out Encoding encoding);
//                if (success)
//                {
//                    Value = encoding;
//                    return true;
//                }
//            }
//            return false;
//        }

//    }
//}