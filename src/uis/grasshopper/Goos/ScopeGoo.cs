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
    public class ScopeGoo : SemioGoo<Scope>
    {
        public ScopeGoo()
        {
            Value = new Scope();
        }

        public ScopeGoo(Scope scope)
        {
            Value = scope;
        }
        public override IGH_Goo Duplicate() => new ScopeGoo(Value.Clone());
        public override string TypeName => Scope.Descriptor.FullName;
        public override string TypeDescription => Scope.Descriptor.Declaration.LeadingComments;
    }
}