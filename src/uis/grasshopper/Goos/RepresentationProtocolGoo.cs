// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;

namespace Semio.UI.Grasshopper.Goos
{
    public class RepresentationProtocolGoo : SemioGoo<RepresentationProtocol>
    {
        public RepresentationProtocolGoo()
        {
            Value = new RepresentationProtocol();
        }

        public RepresentationProtocolGoo(RepresentationProtocol attraction)
        {
            Value = attraction;
        }

        public override IGH_Goo Duplicate() => new RepresentationProtocolGoo(Value);
        public override string TypeName => "RepresentationProtocol";
        public override string TypeDescription => "";
        public override bool CastTo<Q>(ref Q target)
        {
            if (typeof(Q).IsAssignableFrom(typeof(RepresentationProtocol)))
            {
                object ptr = Value;
                target = (Q)ptr;
                return true;
            }
            return false;
        }

        public override bool CastFrom(object source)
        {
            if (source == null) { return false; }

            if (GH_Convert.ToString(source, out var representationProtocolValue, GH_Conversion.Both))
            {
                bool success = Enum.TryParse(representationProtocolValue, true, out RepresentationProtocol representationProtocol);
                if (success)
                {
                    Value = representationProtocol;
                    return true;
                }
            }
            return false;
        }

    }
}