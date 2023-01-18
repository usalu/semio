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
    public class AttractionGoo : GH_Goo<Attraction>
    {
        public AttractionGoo()
        {
            Value = new Attraction();
        }

        public AttractionGoo(Attraction attraction)
        {
            Value = attraction;
        }

        public override IGH_Goo Duplicate() => new AttractionGoo(Value.Clone());

        public override string ToString() => Value.ToString();

        public override bool IsValid => true;
        public override string TypeName => Attraction.Descriptor.FullName;
        public override string TypeDescription => Attraction.Descriptor.Declaration.LeadingComments;
    }
}