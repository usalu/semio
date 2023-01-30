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
    public class PlanGoo : SemioGoo<Plan>
    {
        public PlanGoo()
        {
            Value = new Plan();
        }

        public PlanGoo(Plan assembly)
        {
            Value = assembly;
        }
        public override IGH_Goo Duplicate() => new PlanGoo(Value.Clone());
        public override string TypeName => Plan.Descriptor.FullName;
        public override string TypeDescription => Plan.Descriptor.Declaration.LeadingComments;
    }
}