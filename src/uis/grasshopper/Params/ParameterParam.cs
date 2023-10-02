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
    public class ParameterParam : SemioPersistentParam<ParameterGoo>
    {
        public ParameterParam() :
            base("Parameter", "Pr", "", "semio", "Model")
        { }
        public override Guid ComponentGuid => new("12DE6BA6-4966-45E9-B057-DEFAB51A5BB0");
        protected override GH_GetterResult Prompt_Singular(ref ParameterGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<ParameterGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_parameter;
    }
}