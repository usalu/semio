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
    public class RepresentationProtocolParam : GH_PersistentParam<RepresentationProtocolGoo>
    {
        public RepresentationProtocolParam() :
            base("Representation Protocol", "RP", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("AE160EAD-1FBD-45C6-B1F0-9EE73636D9AB");
        protected override GH_GetterResult Prompt_Singular(ref RepresentationProtocolGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<RepresentationProtocolGoo> values)
        {
            throw new NotImplementedException();
        }
        public override GH_Exposure Exposure => GH_Exposure.primary;
        protected override Bitmap Icon => Resources.icon_representationprotocol;
    }
}