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
    public class PrototypeParam : SemioPersistentParam<PrototypeGoo>
    {
        public PrototypeParam() :
            base("Prototype", "PT", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("E32333D1-D7DD-4B08-AA16-9BBCA3ADFF75");
        protected override GH_GetterResult Prompt_Singular(ref PrototypeGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<PrototypeGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_prototype;
    }
}