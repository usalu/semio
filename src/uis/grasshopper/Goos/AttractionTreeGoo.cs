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
    public class AssemblyGoo : SemioGoo<Assembly>
    {
        public AssemblyGoo()
        {
            Value = new Assembly();
        }

        public AssemblyGoo(Assembly connection)
        {
            Value = connection;
        }
        public override IGH_Goo Duplicate() => new AssemblyGoo(Value.Clone());
        public override string TypeName => Assembly.Descriptor.FullName;
        public override string TypeDescription => Assembly.Descriptor.Declaration.LeadingComments;
    }
}