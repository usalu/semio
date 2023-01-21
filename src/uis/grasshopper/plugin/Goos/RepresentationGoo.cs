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
    public class RepresentationGoo : GH_Goo<Representation>
    {
        public RepresentationGoo()
        {
            Value = new Representation();
        }

        public RepresentationGoo(Representation attraction)
        {
            Value = attraction;
        }

        public override IGH_Goo Duplicate() => new RepresentationGoo(Value.Clone());

        public override string ToString() => Value.ToString();

        public override bool IsValid => true;
        public override string TypeName => Representation.Descriptor.FullName;
        public override string TypeDescription => Representation.Descriptor.Declaration.LeadingComments;
    }
}