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
    public class ConnectableGoo : SemioGoo<Connectable>
    {
        public ConnectableGoo()
        {
            Value = new Connectable();
        }

        public ConnectableGoo(Connectable connectable)
        {
            Value = connectable;
        }

        public override IGH_Goo Duplicate() => new ConnectableGoo(Value.Clone());
        public override string TypeName => Connectable.Descriptor.FullName;
        public override string TypeDescription => Connectable.Descriptor.Declaration.LeadingComments;
    }
}