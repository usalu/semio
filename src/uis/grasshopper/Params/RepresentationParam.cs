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
    public class RepresentationParam : SemioPersistentParam<RepresentationGoo>
    {
        public RepresentationParam() :
            base("Representation", "R", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("9B039F8F-AC2D-4079-8D5A-675350861B9C");
        protected override GH_GetterResult Prompt_Singular(ref RepresentationGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<RepresentationGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_connection;
    }
}