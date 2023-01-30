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
    public class PrototypeGoo : SemioGoo<Prototype>
    {
        public PrototypeGoo()
        {
            Value = new Prototype();
        }

        public PrototypeGoo(Prototype prototype)
        {
            Value = prototype;
        }
        public override IGH_Goo Duplicate() => new PrototypeGoo(Value.Clone());
        public override string TypeName => Prototype.Descriptor.FullName;
        public override string TypeDescription => Prototype.Descriptor.Declaration.LeadingComments;
    }
}