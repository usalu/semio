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
    public class LinkGoo : SemioGoo<Link>
    {
        public LinkGoo()
        {
            Value = new Link();
        }

        public LinkGoo(Link link)
        {
            Value = link;
        }
        public override IGH_Goo Duplicate() => new LinkGoo(Value.Clone());
        public override string TypeName => Link.Descriptor.FullName;
        public override string TypeDescription => Link.Descriptor.Declaration.LeadingComments;
    }
}