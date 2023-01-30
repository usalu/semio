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
    public class FileTypeParam : SemioPersistentParam<FileTypeGoo>
    {
        public FileTypeParam() :
            base("File Type", "FT", "", "Semio", "Model")
        { }
        public override Guid ComponentGuid => new("E52EADC4-1702-4032-867B-7B78616D17B2");
        protected override GH_GetterResult Prompt_Singular(ref FileTypeGoo value)
        {
            throw new NotImplementedException();
        }
        protected override GH_GetterResult Prompt_Plural(ref List<FileTypeGoo> values)
        {
            throw new NotImplementedException();
        }
        protected override Bitmap Icon => Resources.icon_filetype;
    }
}