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
    public class LayoutParam : SemioPersistentParam<LayoutGoo>
    {
        public LayoutParam() :
            base("Layout", "L", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("4D8B21C7-8F20-4BC2-B6FB-6DDC058B27F1");
        protected override GH_GetterResult Prompt_Singular(ref LayoutGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<LayoutGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_layout;
    }
}