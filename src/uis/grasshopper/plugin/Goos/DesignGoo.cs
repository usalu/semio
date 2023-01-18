// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;

namespace Semio.UI.Grasshopper.Goos
{
    public class DesignGoo : GH_Goo<Design>
    {
        public DesignGoo()
        {
            Value = new Design();
        }

        public DesignGoo(Design design)
        {
            Value = design;
        }

        public override IGH_Goo Duplicate() => new DesignGoo(Value.Clone());

        public override string ToString() => Value.ToString();

        public override bool IsValid => true;
        public override string TypeName => Design.Descriptor.FullName;
        public override string TypeDescription => Design.Descriptor.Declaration.LeadingComments;
    }
}