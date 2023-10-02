// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper.Params
{
    public class DesignParam : SemioPersistentParam<DesignGoo>
    {
        public DesignParam() :
            base("Design", "D", "", "semio", "Model")
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
        protected override Bitmap Icon => Resources.icon_design;
    }
}