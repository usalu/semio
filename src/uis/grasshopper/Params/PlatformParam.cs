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
    public class PlatformParam : SemioPersistentParam<PlatformGoo>
    {
        public PlatformParam() :
            base("Platform", "Pf", "", "semio", "Model")
        { }
        public override Guid ComponentGuid => new("7DA7E9B0-3FE0-4E85-A615-A7196E6E073E");
        protected override GH_GetterResult Prompt_Singular(ref PlatformGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<PlatformGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_platform;
    }
}