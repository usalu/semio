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
    public class LayoutStrategyParam : GH_PersistentParam<LayoutStrategyGoo>
    {
        public LayoutStrategyParam() :
            base("Layout Strategy", "RP", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("9F6C33F9-EAF5-40A5-9C27-B5B8110A87EC");
        protected override GH_GetterResult Prompt_Singular(ref LayoutStrategyGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<LayoutStrategyGoo> values)
        {
            throw new NotImplementedException();
        }
        public override GH_Exposure Exposure => GH_Exposure.primary;
        protected override Bitmap Icon => Resources.icon_layoutstrategy;
    }
}