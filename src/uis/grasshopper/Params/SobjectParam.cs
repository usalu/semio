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
    public class SobjectParam : SemioPersistentParam<SobjectGoo>
    {
        public SobjectParam() :
            base("Sobject", "So", "", "semio", "Model")
        { }
        public override Guid ComponentGuid => new("A6B3D7AF-5B3D-445E-8F33-9B2A28DA1D22");
        protected override GH_GetterResult Prompt_Singular(ref SobjectGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<SobjectGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_sobject;
    }
}