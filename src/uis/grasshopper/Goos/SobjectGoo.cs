// TODO Autogenerate
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Security.Policy;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;
using Google.Protobuf;
using Grasshopper.GUI;
using Grasshopper.Kernel.Types;
using Rhino.Geometry;
using Semio.Model.V1;
using Encoding = System.Text.Encoding;

namespace Semio.UI.Grasshopper.Goos
{
    public class SobjectGoo: SemioGoo<Sobject>
    {
        public SobjectGoo()
        {
            Value = new Sobject();
        }

        public SobjectGoo(Sobject sobject)
        {
            Value=sobject;
        }
        public override IGH_Goo Duplicate() => new SobjectGoo(Value.Clone());
        public override string TypeName => Sobject.Descriptor.FullName;
        public override string TypeDescription => Sobject.Descriptor.Declaration.LeadingComments;

        public override bool CastFrom(object source)
        {
            if (source == null) return false;

            switch (source)
            {
                case GH_String text:
                    try
                    {
                        Value = Sobject.Parser.ParseJson(text.Value);
                        return true;

                    }
                    catch (Exception e)
                    {
                        try
                        {
                            Value = Sobject.Parser.ParseFrom(ByteString.CopyFrom(text.Value, Encoding.Default));
                            return true;
                        }
                        catch (Exception exception)
                        {
                          
                        }
                    }
                    return false;
                default:
                    return false;
            }
        }
    }
}
