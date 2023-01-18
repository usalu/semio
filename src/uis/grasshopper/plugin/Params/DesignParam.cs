// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Grasshopper.Kernel;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
namespace Semio.UI.Grasshopper.Params
{
    public class DesignParam : GH_PersistentParam<DesignGoo>
    {
        public DesignParam() :
            base(Design.Descriptor.Name, Design.Descriptor.Name, Design.Descriptor.Declaration.LeadingComments, "Semio", Design.Descriptor.File.Name.ToUpper())
        { }
        public override Guid ComponentGuid => new("BBDA367A-2A30-4B6D-B36E-54DC01A78037");
        protected override GH_GetterResult Prompt_Singular(ref DesignGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<DesignGoo> values)
        {
            throw new NotImplementedException();
        }
        public override GH_Exposure Exposure => GH_Exposure.primary;
    }
}