// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Grasshopper.Kernel;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Params
{
    public class PlanParam : SemioPersistentParam<PlanGoo>
    {
        public PlanParam() :
            base("Plan", "Pl", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("87F6F9AC-F2AD-45E1-A5EC-AD666A20C3B8");
        protected override GH_GetterResult Prompt_Singular(ref PlanGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<PlanGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_assembly;
    }
}