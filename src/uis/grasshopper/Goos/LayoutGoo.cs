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
    public class LayoutGoo : SemioGoo<Layout>
    {
        public LayoutGoo()
        {
            Value = new Layout();
        }

        public LayoutGoo(Layout layout)
        {
            Value = layout;
        }

        public override IGH_Goo Duplicate() => new LayoutGoo(Value.Clone());
        public override string TypeName => Layout.Descriptor.FullName;
        public override string TypeDescription => Layout.Descriptor.Declaration.LeadingComments;
    }
}