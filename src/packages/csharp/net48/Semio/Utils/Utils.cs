using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Google.Protobuf;

namespace Semio.Utils
{
    public static class Utils
    {
        public static byte[] ToBytes(dynamic semioObject) => ((IMessage)semioObject).ToByteArray();

        public static string ToByteString(dynamic semioObject) => ByteString.CopyFrom(ToBytes(semioObject)).ToStringUtf8();
        public static string ToBase64(dynamic semioObject) => Convert.ToBase64String(ToBytes(semioObject));
    }
}
