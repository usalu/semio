// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Grasshopper.Kernel;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
namespace Semio.UI.Grasshopper.Params
{
    public class AttractionParam : GH_PersistentParam<AttractionGoo>
    {
        public AttractionParam() :
            base("Attraction", "A", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("745B3720-2333-44F1-9B81-A41DAA7A894F");
        protected override GH_GetterResult Prompt_Singular(ref AttractionGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<AttractionGoo> values)
        {
            throw new NotImplementedException();
        }
        public override GH_Exposure Exposure => GH_Exposure.primary;
    }
}