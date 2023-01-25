using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;

namespace Semio.UI.Grasshopper.Params
{
    public abstract class SemioPersistentParam<T> : GH_PersistentParam<T> where T : class, IGH_Goo
    {
        protected SemioPersistentParam(string name, string nickname, string description, string category, string subcategory) : base(name, nickname, description, category, subcategory)
        {
        }
        public override GH_Exposure Exposure => GH_Exposure.primary;
    }
}
