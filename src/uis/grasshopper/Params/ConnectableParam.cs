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
    public class ConnectableParam : SemioPersistentParam<ConnectableGoo>
    {
        public ConnectableParam() :
            base("Connection Participant", "AP", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("61BEC70C-3BDB-4824-92FB-03E053253166");
        protected override GH_GetterResult Prompt_Singular(ref ConnectableGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<ConnectableGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_connectable;
    }
}