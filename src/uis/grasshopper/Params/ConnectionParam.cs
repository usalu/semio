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
    public class ConnectionParam : SemioPersistentParam<ConnectionGoo>
    {
        public ConnectionParam() :
            base("Connection", "A", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("745B3720-2333-44F1-9B81-A41DAA7A894F");
        protected override GH_GetterResult Prompt_Singular(ref ConnectionGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<ConnectionGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_connection;
    }
}