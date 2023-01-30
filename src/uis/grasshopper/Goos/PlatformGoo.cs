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
    public class PlatformGoo : SemioGoo<Platform>
    {
        public PlatformGoo()
        {
            Value = new Platform();
        }
        public PlatformGoo(Platform platform)
        {
            Value = platform;
        }
        public override IGH_Goo Duplicate() => new PlatformGoo(Value);
        public override string TypeName => "Platform";
        public override string TypeDescription => "";
        public override bool CastTo<Q>(ref Q target)
        {
            if (typeof(Q).IsAssignableFrom(typeof(Platform)))
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

            if (GH_Convert.ToString(source, out var platformValue, GH_Conversion.Both))
            {
                bool success = Enum.TryParse(platformValue, true, out Platform platform);
                if (success)
                {
                    Value = platform;
                    return true;
                }
            }
            return false;
        }

    }
}