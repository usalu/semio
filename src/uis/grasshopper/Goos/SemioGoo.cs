using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Semio.Model.V1;

namespace Semio.UI.Grasshopper.Goos
{
    public abstract class SemioGoo<T> : GH_Goo<T>
    {
        public override string ToString() => Value.ToString();
        public override bool IsValid => true;
    }
}
