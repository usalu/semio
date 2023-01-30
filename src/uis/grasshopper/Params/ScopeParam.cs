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
    public class ScopeParam : SemioPersistentParam<ScopeGoo>
    {
        public ScopeParam() :
            base("Scope", "Sc", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("1750584C-69E0-46E9-8194-F2D41B977E7B");
        protected override GH_GetterResult Prompt_Singular(ref ScopeGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<ScopeGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_scope;
    }
}