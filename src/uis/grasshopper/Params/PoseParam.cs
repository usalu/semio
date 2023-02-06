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
    public class PoseParam : SemioPersistentParam<PoseGoo>
    {
        public PoseParam() :
            base("Pose", "Po", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("FCADBFCE-C1C4-465B-BA12-B03132C0F258");

        protected override GH_GetterResult Prompt_Singular(ref PoseGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<PoseGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_pose;
    }
}