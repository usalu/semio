// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;

namespace Semio.UI.Grasshopper.Goos
{
    public class FileTypeGoo : SemioGoo<FileType>
    {
        public FileTypeGoo()
        {
            Value = new FileType();
        }
        public FileTypeGoo(FileType fileType)
        {
            Value = fileType;
        }
        public override IGH_Goo Duplicate() => new FileTypeGoo(Value);
        public override string TypeName => "FileType";
        public override string TypeDescription => "";
        public override bool CastTo<Q>(ref Q target)
        {
            if (typeof(Q).IsAssignableFrom(typeof(FileType)))
            {
                object ptr = Value;
                target = (Q)ptr;
                return true;
            }
            return false;
        }

        public override bool CastFrom(object source)
        {
            if (source == null) { return false; }

            if (GH_Convert.ToString(source, out var fileTypeValue, GH_Conversion.Both))
            {
                bool success = Enum.TryParse(fileTypeValue, true, out FileType fileType);
                if (success)
                {
                    Value = fileType;
                    return true;
                }
            }
            return false;
        }

    }
}