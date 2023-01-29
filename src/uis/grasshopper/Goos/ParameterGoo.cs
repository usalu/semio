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
    public class ParameterGoo : SemioGoo<Parameter>
    {
        public ParameterGoo()
        {
            Value = new Parameter();
        }

        public ParameterGoo(Parameter parameter)
        {
            Value = parameter;
        }

        public override IGH_Goo Duplicate() => new ParameterGoo(Value.Clone());
        public override string TypeName => Parameter.Descriptor.FullName;
        public override string TypeDescription => Parameter.Descriptor.Declaration.LeadingComments;
    }
}