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
    public class ElementGoo : SemioGeometricGoo<Element>
    {
        public ElementGoo()
        {
            Value = new Element();
        }
        public ElementGoo(Element element)
        {
            Value = element;
        }
        public override IGH_Goo Duplicate() => new ElementGoo(Value.Clone());
        public override string TypeName => Element.Descriptor.FullName;
        public override string TypeDescription => Element.Descriptor.Declaration.LeadingComments;
    }
}