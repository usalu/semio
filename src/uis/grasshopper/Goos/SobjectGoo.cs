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
    public class SobjectGoo: SemioGoo<Sobject>
    {
        public SobjectGoo()
        {
            Value = new Sobject();
        }

        public SobjectGoo(Sobject sobject)
        {
            Value=sobject;
        }
        public override IGH_Goo Duplicate() => new SobjectGoo(Value.Clone());
        public override string TypeName => Sobject.Descriptor.FullName;
        public override string TypeDescription => Sobject.Descriptor.Declaration.LeadingComments;
    }
}
