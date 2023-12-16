using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;

namespace Semio.Grasshopper.Params;

public abstract class SemioPersistentParam<T> : GH_PersistentParam<T> where T : class, IGH_Goo
{
    protected SemioPersistentParam(string name, string nickname, string description, string category,
        string subcategory) : base(name, nickname, description, category, subcategory)
    {
    }
}