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
    public class ElementParam : SemioPersistentParam<ElementGoo>
    {
        public ElementParam() :
            base("Element", "E", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("2507E825-CB09-4268-83FC-454114B39DB1");
        protected override GH_GetterResult Prompt_Singular(ref ElementGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<ElementGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_element;
    }
}