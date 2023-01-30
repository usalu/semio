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
    public class ValueParam : SemioPersistentParam<ValueGoo>
    {
        public ValueParam() :
            base("Value", "V", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("CC16A3CA-682D-4467-B029-DBAC103A1782");
        protected override GH_GetterResult Prompt_Singular(ref ValueGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<ValueGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_value;
    }
}