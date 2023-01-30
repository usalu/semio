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
    public class LayoutStrategyGoo : SemioGoo<LayoutStrategy>
    {
        public LayoutStrategyGoo()
        {
            Value = new LayoutStrategy();
        }
        public LayoutStrategyGoo(LayoutStrategy layoutStrategy)
        {
            Value = layoutStrategy;
        }
        public override IGH_Goo Duplicate() => new LayoutStrategyGoo(Value);
        public override string TypeName => "LayoutStrategy";
        public override string TypeDescription => "";
        public override bool CastTo<Q>(ref Q target)
        {
            if (typeof(Q).IsAssignableFrom(typeof(LayoutStrategy)))
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

            if (GH_Convert.ToString(source, out var layoutStrategyValue, GH_Conversion.Both))
            {
                bool success = Enum.TryParse(layoutStrategyValue, true, out LayoutStrategy layoutStrategy);
                if (success)
                {
                    Value = layoutStrategy;
                    return true;
                }
            }
            return false;
        }

    }
}