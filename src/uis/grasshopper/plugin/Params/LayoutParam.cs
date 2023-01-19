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
    public class LayoutParam : GH_PersistentParam<LayoutGoo>
    {
        public LayoutParam() :
            base("Layout", "A", "", "Semio", "Model")
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
        public override GH_Exposure Exposure => GH_Exposure.primary;
        protected override Bitmap Icon => Resources.icon_layout;
    }
}