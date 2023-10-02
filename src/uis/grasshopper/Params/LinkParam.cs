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
    public class LinkParam : SemioPersistentParam<LinkGoo>
    {
        public LinkParam() :
            base("Link", "Lk", "", "semio", "Model")
        { }
        public override Guid ComponentGuid => new("F55CFEF8-4555-41B0-AA80-1697E955A3CD");
        protected override GH_GetterResult Prompt_Singular(ref LinkGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<LinkGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_link;
    }
}