using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Grasshopper.Kernel.Types;

namespace Semio.UI.Grasshopper.Goos
{
    public abstract class SemioGeometricGoo<T> : GH_GeometricGoo<T>
    {
        public override string ToString() => Value.ToString();
        public override bool IsValid => true;
    }
}
