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
    public class AttractionTreeGoo : SemioGoo<AttractionTree>
    {
        public AttractionTreeGoo()
        {
            Value = new AttractionTree();
        }

        public AttractionTreeGoo(AttractionTree attraction)
        {
            Value = attraction;
        }
        public override IGH_Goo Duplicate() => new AttractionTreeGoo(Value.Clone());
        public override string TypeName => AttractionTree.Descriptor.FullName;
        public override string TypeDescription => AttractionTree.Descriptor.Declaration.LeadingComments;
    }
}